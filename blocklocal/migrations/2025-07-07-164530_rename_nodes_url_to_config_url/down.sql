-- This file should undo anything in `up.sql`
ALTER TABLE "nodes" DROP COLUMN "config_url";
ALTER TABLE "nodes" ADD COLUMN "url" TEXT NOT NULL;

