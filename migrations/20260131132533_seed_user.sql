-- Add admin user
INSERT INTO users (user_id, username, password_hash)
VALUES (gen_random_uuid(),
        'admin',
        '$argon2id$v=19$m=15000,t=2,p=1$Ef/hWDblW3kbV41wGGk3/Q$BR4wYKaKJknDNtCNk3gYkyS4gwXBAMTyrqDgq7Bgfi8')