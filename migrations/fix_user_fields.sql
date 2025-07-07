-- 更新用户表的email和password字段长度限制
-- 这个脚本用于修复管理员账户创建时的字段长度问题

-- 修改email字段长度为255个字符（支持更长的邮箱地址）
ALTER TABLE purple_user ALTER COLUMN email TYPE varchar(255);

-- 修改password字段长度为255个字符（支持Argon2哈希后的密码）
ALTER TABLE purple_user ALTER COLUMN password TYPE varchar(255);

-- 添加token字段（如果不存在）
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='purple_user' AND column_name='token') THEN
        ALTER TABLE purple_user ADD COLUMN token varchar(255);
    END IF;
END $$;

-- 添加created_at和updated_at字段（如果不存在）
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='purple_user' AND column_name='created_at') THEN
        ALTER TABLE purple_user ADD COLUMN created_at integer NOT NULL DEFAULT extract(epoch from now())::integer;
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='purple_user' AND column_name='updated_at') THEN
        ALTER TABLE purple_user ADD COLUMN updated_at integer NOT NULL DEFAULT extract(epoch from now())::integer;
    END IF;
END $$;