# Algorithmite Website

Algorithmite is a full stack web application for the Algorithmite Youtube channel created on livestreams at the Algorithmite Twitch channel

It is made using Rust as the back end and html templating with some javascript for the front end
Rocket 0.5-rc as the server for stable rust support and simpler built in extensions
Diesel as the ORM around Postgres as the database
Handlebars for templating

algorithmite-website/src is the primary web application running on the server
algorithmite-website/www is the base folder for all main templates and styles for the website
algorithmite-website/sql is the base folder for database migrations
