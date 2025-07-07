-- Your SQL goes here
ALTER TABLE "nodes" DROP COLUMN "url";
ALTER TABLE "nodes" ADD COLUMN "config_url" TEXT NOT NULL;

