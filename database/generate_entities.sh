sea-orm-cli generate entity \
    -u postgresql://"$DB_USERNAME":"$DB_PASSWORD"@"$DB_HOST":5432/"$DB_NAME" \
    -o src/entities