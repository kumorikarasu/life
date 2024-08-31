# npm dev

from node:20 as dev

WORKDIR /app

COPY package.json /app
COPY package-lock.json /app
RUN npm install

COPY . /app

cmd ["npm", "run", "dev"]

# npm build
from node:20 as build

WORKDIR /app

COPY package.json /app
COPY package-lock.json /app
RUN npm install

COPY . /app
RUN npm run build -prod

from nginx:1.19.10-alpine as prod
copy --from=build /app/build /var/www
copy ./nginx.conf /etc/nginx/nginx.conf
