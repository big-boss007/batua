ALTER TABLE referral_programs
ADD COLUMN code_creation_trigger TEXT NOT NULL DEFAULT 'on_registration';
