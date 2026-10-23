import React from 'react';
import { useTranslation } from '../../../i18n';
import { TriggerProfile } from '../../../types/models';

interface Props {
  profiles: TriggerProfile[];
  onApplyProfile: (profileId: string) => void;
}

export const GameProfilesStudio: React.FC<Props> = ({ profiles, onApplyProfile }) => {
  const { t } = useTranslation();

  return (
    <div className="profiles-studio-container">
      <div className="studio-header">
        <div>
          <h3>{t('dualsense_profiles_title')}</h3>
          <p className="section-subtitle">{t('dualsense_profiles_subtitle')}</p>
        </div>
      </div>

      <div className="profiles-grid">
        {profiles.map((profile) => (
          <div key={profile.id} className="deck-card profile-card">
            <div className="profile-card-top">
              <h4>{profile.name}</h4>
              <div
                className="profile-color-chip"
                style={{
                  backgroundColor: `rgb(${profile.lightbar.r}, ${profile.lightbar.g}, ${profile.lightbar.b})`,
                }}
              />
            </div>
            <p className="card-desc">{profile.description}</p>
            <div className="profile-details-preview">
              <span className="badge-tag">L2: {profile.triggers.left.mode.toUpperCase()}</span>
              <span className="badge-tag">R2: {profile.triggers.right.mode.toUpperCase()}</span>
            </div>
            <button
              className="deck-btn primary apply-profile-btn"
              onClick={() => onApplyProfile(profile.id)}
            >
              🎮 {t('dualsense_apply_profile')}
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};

