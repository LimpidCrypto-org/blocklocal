-- This file should undo anything in `up.sql`
ALTER TABLE "blockchains" DROP COLUMN "config_url";
ALTER TABLE "blockchains" ADD COLUMN "url" TEXT NOT NULL;

