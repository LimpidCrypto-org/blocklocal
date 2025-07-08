-- Your SQL goes here
ALTER TABLE "blockchains" DROP COLUMN "url";
ALTER TABLE "blockchains" ADD COLUMN "config_url" TEXT NOT NULL;

