import { DualTriggerConfig, LightingConfig, TriggerEffect } from '../types/models';

export const SONY_VENDOR_ID = 0x054c;
export const DUALSENSE_PID = 0x0ce6;
export const DUALSENSE_EDGE_PID = 0x0df2;
export const DUALSHOCK4_V1_PID = 0x05c4;
export const DUALSHOCK4_V2_PID = 0x09cc;

export interface LiveGamepadState {
  connected: boolean;
  id: string;
  leftStick: { x: number; y: number };
  rightStick: { x: number; y: number };
  l2: number;
  r2: number;
  buttons: {
    cross: boolean;
    circle: boolean;
    square: boolean;
    triangle: boolean;
    l1: boolean;
    r1: boolean;
    l2: boolean;
    r2: boolean;
    create: boolean;
    options: boolean;
    l3: boolean;
    r3: boolean;
    dpadUp: boolean;
    dpadDown: boolean;
    dpadLeft: boolean;
    dpadRight: boolean;
    ps: boolean;
    touchpad: boolean;
  };
}

export function compileTriggerBytes(effect: TriggerEffect): Uint8Array {
  const bytes = new Uint8Array(11);
  switch (effect.mode) {
    case 'feedback':
      bytes[0] = 0x21; // Feedback mode
      bytes[1] = Math.min(Math.max(effect.start_position, 0), 9);
      bytes[2] = Math.min(Math.max(effect.force, 0), 8);
      break;
    case 'weapon':
      bytes[0] = 0x25; // Weapon mode
      bytes[1] = Math.min(Math.max(effect.start_position, 0), 9);
      bytes[2] = Math.min(Math.max(effect.end_position, 0), 9);
      bytes[3] = Math.min(Math.max(effect.force, 0), 8);
      break;
    case 'bow':
      bytes[0] = 0x22; // Bow mode
      bytes[1] = Math.min(Math.max(effect.start_position, 0), 9);
      bytes[2] = Math.min(Math.max(effect.end_position, 0), 9);
      bytes[3] = Math.min(Math.max(effect.force, 0), 8);
      bytes[4] = Math.min(Math.max(effect.snap_force, 0), 8);
      break;
    case 'machine_gun':
      bytes[0] = 0x26; // Galloping / Machine gun
      bytes[1] = Math.min(Math.max(effect.start_position, 0), 9);
      bytes[2] = Math.min(Math.max(effect.end_position, 0), 9);
      bytes[3] = Math.min(Math.max(effect.strength, 0), 8);
      bytes[4] = Math.min(Math.max(effect.frequency, 1), 40);
      break;
    case 'vibration':
      bytes[0] = 0x23; // Vibration mode
      bytes[1] = Math.min(Math.max(effect.position, 0), 9);
      bytes[2] = Math.min(Math.max(effect.amplitude, 0), 8);
      bytes[3] = Math.min(Math.max(effect.frequency, 1), 40);
      break;
    case 'off':
    default:
      bytes[0] = 0x00;
      break;
  }
  return bytes;
}

export interface WebHidDevice {
  opened: boolean;
  productName?: string;
  collections?: Array<{
    outputReports?: Array<{ reportId: number }>;
  }>;
  open(): Promise<void>;
  close(): Promise<void>;
  sendReport(reportId: number, data: Uint8Array): Promise<void>;
}

function calculateCrc32(data: Uint8Array): number {
  let crc = 0xffffffff;
  for (let i = 0; i < data.length; i++) {
    crc ^= data[i];
    for (let j = 0; j < 8; j++) {
      crc = (crc >>> 1) ^ (-(crc & 1) & 0xedb88320);
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

export class WebHidDualSenseController {
  private device: WebHidDevice | null = null;
  private isBluetooth = false;
  private btSeq = 0;

  public static isSupported(): boolean {
    return typeof navigator !== 'undefined' && 'hid' in navigator;
  }

  public isConnected(): boolean {
    return this.device !== null && this.device.opened;
  }

  public getDeviceName(): string {
    return this.device?.productName || 'Sony Wireless Controller';
  }

  public async connect(): Promise<boolean> {
    if (!WebHidDualSenseController.isSupported()) {
      return false;
    }

    try {
      const nav = navigator as unknown as {
        hid: {
          requestDevice: (opts: { filters: Array<{ vendorId: number; productId: number }> }) => Promise<WebHidDevice[]>;
        };
      };

      const devices = await nav.hid.requestDevice({
        filters: [
          { vendorId: SONY_VENDOR_ID, productId: DUALSENSE_PID },
          { vendorId: SONY_VENDOR_ID, productId: DUALSENSE_EDGE_PID },
          { vendorId: SONY_VENDOR_ID, productId: DUALSHOCK4_V1_PID },
          { vendorId: SONY_VENDOR_ID, productId: DUALSHOCK4_V2_PID },
        ],
      });

      if (devices.length === 0) return false;

      this.device = devices[0];
      if (!this.device.opened) {
        await this.device.open();
      }

      this.isBluetooth =
        this.device.collections?.some(
          (c) => c.outputReports?.some((r) => r.reportId === 0x31)
        ) || false;

      return true;
    } catch (e) {
      console.warn('Failed to connect via WebHID:', e);
      return false;
    }
  }

  public buildUsbPayload(
    triggers: DualTriggerConfig,
    lighting: LightingConfig,
    rumbleHeavy = 0,
    rumbleLight = 0
  ): Uint8Array {
    const payload = new Uint8Array(63);
    payload[0] = 0xff; // valid_flag0: enable rumble, triggers
    payload[1] = 0x15; // valid_flag1: enable mic led (0x01), lightbar (0x04), player leds (0x10) - NO 0x08 RELEASE_LEDS
    payload[2] = rumbleLight;
    payload[3] = rumbleHeavy;

    // Mic LED
    payload[8] = lighting.mic_led === 'solid' ? 0x01 : lighting.mic_led === 'pulse' ? 0x02 : 0x00;

    // Right Trigger (offset 10 in 63-byte payload, corresponding to offset 11 in 64-byte report)
    const rightBytes = compileTriggerBytes(triggers.right);
    payload.set(rightBytes, 10);

    // Left Trigger (offset 21 in 63-byte payload, corresponding to offset 22 in 64-byte report)
    const leftBytes = compileTriggerBytes(triggers.left);
    payload.set(leftBytes, 21);

    // valid_flag2: DS_OUTPUT_VALID_FLAG2_LIGHTBAR_SETUP_CONTROL_ENABLE
    payload[38] = 0x02;

    // lightbar_setup: 0x02 = DS_OUTPUT_LIGHTBAR_SETUP_LIGHT_OUT (clears default fade so RGB takes over)
    payload[41] = 0x02;

    // led_brightness: 0x00 = High brightness
    payload[42] = 0x00;

    // player_leds
    switch (lighting.player_led) {
      case 'player1': payload[43] = 0x04; break;
      case 'player2': payload[43] = 0x0a; break;
      case 'player3': payload[43] = 0x15; break;
      case 'player4': payload[43] = 0x1b; break;
      case 'all':     payload[43] = 0x1f; break;
      default:        payload[43] = 0x00; break;
    }

    // RGB Lightbar
    const color =
      lighting.lightbar.mode === 'static' || lighting.lightbar.mode === 'breathing'
        ? lighting.lightbar.color
        : lighting.lightbar.mode === 'off'
          ? { r: 0, g: 0, b: 0 }
          : { r: 0, g: 68, b: 204 };

    payload[44] = color.r;
    payload[45] = color.g;
    payload[46] = color.b;

    return payload;
  }

  public buildBluetoothPayload(
    triggers: DualTriggerConfig,
    lighting: LightingConfig,
    rumbleHeavy = 0,
    rumbleLight = 0
  ): Uint8Array {
    const payload = new Uint8Array(77);
    payload[0] = ((this.btSeq & 0x0f) << 4) | 0x02; // Rolling BT sequence tag + 0x02 flag
    this.btSeq = (this.btSeq + 1) & 0x0f;
    payload[1] = 0x10; // DS_OUTPUT_TAG

    // common starts at index 2 of 77-byte BT payload
    payload[2] = 0xff; // valid_flag0
    payload[3] = 0x15; // valid_flag1 (0x01 | 0x04 | 0x10, NO 0x08 RELEASE_LEDS)
    payload[4] = rumbleLight;
    payload[5] = rumbleHeavy;

    // Mic LED
    payload[10] = lighting.mic_led === 'solid' ? 0x01 : lighting.mic_led === 'pulse' ? 0x02 : 0x00;

    // Right Trigger (offset 2 + 10 = 12)
    const rightBytes = compileTriggerBytes(triggers.right);
    payload.set(rightBytes, 12);

    // Left Trigger (offset 2 + 21 = 23)
    const leftBytes = compileTriggerBytes(triggers.left);
    payload.set(leftBytes, 23);

    // valid_flag2 (offset 2 + 38 = 40)
    payload[40] = 0x02;

    // lightbar_setup (offset 2 + 41 = 43)
    payload[43] = 0x02;

    // led_brightness (offset 2 + 42 = 44)
    payload[44] = 0x00;

    // player_leds (offset 2 + 43 = 45)
    switch (lighting.player_led) {
      case 'player1': payload[45] = 0x04; break;
      case 'player2': payload[45] = 0x0a; break;
      case 'player3': payload[45] = 0x15; break;
      case 'player4': payload[45] = 0x1b; break;
      case 'all':     payload[45] = 0x1f; break;
      default:        payload[45] = 0x00; break;
    }

    // RGB Lightbar (offset 2 + 44 = 46..48)
    const color =
      lighting.lightbar.mode === 'static' || lighting.lightbar.mode === 'breathing'
        ? lighting.lightbar.color
        : lighting.lightbar.mode === 'off'
          ? { r: 0, g: 0, b: 0 }
          : { r: 0, g: 68, b: 204 };

    payload[46] = color.r;
    payload[47] = color.g;
    payload[48] = color.b;

    // Calculate CRC32 over [0xA2, 0x31, ...payload[0..73]]
    const crcBuf = new Uint8Array(75);
    crcBuf[0] = 0xa2;
    crcBuf[1] = 0x31;
    crcBuf.set(payload.subarray(0, 73), 2);
    const checksum = calculateCrc32(crcBuf);

    // Write Little-Endian CRC32 into payload[73..77]
    payload[73] = checksum & 0xff;
    payload[74] = (checksum >>> 8) & 0xff;
    payload[75] = (checksum >>> 16) & 0xff;
    payload[76] = (checksum >>> 24) & 0xff;

    return payload;
  }

  public async sendReport(
    triggers: DualTriggerConfig,
    lighting: LightingConfig,
    rumbleHeavy = 0,
    rumbleLight = 0
  ): Promise<void> {
    if (!this.device || !this.device.opened) return;

    try {
      if (this.isBluetooth) {
        const payload = this.buildBluetoothPayload(triggers, lighting, rumbleHeavy, rumbleLight);
        await this.device.sendReport(0x31, payload);
      } else {
        const payload = this.buildUsbPayload(triggers, lighting, rumbleHeavy, rumbleLight);
        try {
          await this.device.sendReport(0x02, payload);
        } catch (usbErr) {
          console.debug('USB report failed, retrying Bluetooth report format:', usbErr);
          const btPayload = this.buildBluetoothPayload(triggers, lighting, rumbleHeavy, rumbleLight);
          await this.device.sendReport(0x31, btPayload);
          this.isBluetooth = true;
        }
      }
    } catch (e) {
      console.warn('WebHID sendReport failed:', e);
    }
  }

  public async disconnect(): Promise<void> {
    if (this.device && this.device.opened) {
      await this.device.close();
    }
    this.device = null;
  }
}

export const webHidController = new WebHidDualSenseController();

export function pollStandardGamepad(): LiveGamepadState | null {
  if (typeof navigator === 'undefined' || !navigator.getGamepads) {
    return null;
  }

  const gamepads = navigator.getGamepads();
  let gp: Gamepad | null = null;
  for (let i = 0; i < gamepads.length; i++) {
    const candidate = gamepads[i];
    if (candidate && candidate.connected) {
      gp = candidate;
      break;
    }
  }

  if (!gp) return null;

  const btn = (index: number) => gp?.buttons[index]?.pressed || false;
  const btnVal = (index: number) => gp?.buttons[index]?.value || 0;

  return {
    connected: true,
    id: gp.id,
    leftStick: {
      x: Math.round((gp.axes[0] || 0) * 100) / 100,
      y: Math.round((gp.axes[1] || 0) * 100) / 100,
    },
    rightStick: {
      x: Math.round((gp.axes[2] || 0) * 100) / 100,
      y: Math.round((gp.axes[3] || 0) * 100) / 100,
    },
    l2: Math.round(btnVal(6) * 100),
    r2: Math.round(btnVal(7) * 100),
    buttons: {
      cross: btn(0),
      circle: btn(1),
      square: btn(2),
      triangle: btn(3),
      l1: btn(4),
      r1: btn(5),
      l2: btn(6),
      r2: btn(7),
      create: btn(8),
      options: btn(9),
      l3: btn(10),
      r3: btn(11),
      dpadUp: btn(12),
      dpadDown: btn(13),
      dpadLeft: btn(14),
      dpadRight: btn(15),
      ps: btn(16),
      touchpad: btn(17),
    },
  };
}

export async function triggerHapticRumble(duration = 400, strong = 0.8, weak = 0.8): Promise<boolean> {
  if (typeof navigator === 'undefined' || !navigator.getGamepads) return false;

  const gamepads = navigator.getGamepads();
  for (let i = 0; i < gamepads.length; i++) {
    const gp = gamepads[i];
    if (gp && gp.connected && (gp as unknown as { vibrationActuator?: { playEffect: (type: string, opts: object) => Promise<unknown> } }).vibrationActuator) {
      try {
        const actuator = (gp as unknown as { vibrationActuator: { playEffect: (type: string, opts: object) => Promise<unknown> } }).vibrationActuator;
        await actuator.playEffect('dual-rumble', {
          startDelay: 0,
          duration,
          weakMagnitude: weak,
          strongMagnitude: strong,
        });
        return true;
      } catch (e) {
        console.debug('Gamepad haptic rumble call failed:', e);
      }
    }
  }
  return false;
}
