-- Your SQL goes here

INSERT INTO owners (id, user_id, name, description, types, secret_key, service_key, is_active)
VALUES (1, 1, 'Браузерное приложение', 'general app', 1, '%n%#Nv!|y9nU', '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', true ) ON CONFLICT DO NOTHING;
