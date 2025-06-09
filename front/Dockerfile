# npm dev
FROM node:20 as dev

WORKDIR /app

COPY package.json /app
COPY package-lock.json /app
RUN npm install

COPY . /app

CMD ["npm", "run", "dev"]

# npm build
FROM node:20 as build

WORKDIR /app

ENV PORT=80

COPY package.json /app
COPY package-lock.json /app
RUN npm ci

COPY . /app
RUN npm run build

FROM nginx:1.19.10-alpine as prod

RUN addgroup -g 1001 -S nodejs && \
    adduser -S frontend -u 1001 -G nodejs

RUN mkdir -p /var/www /var/cache/nginx /tmp && \
    chown -R frontend:nodejs /var/www /var/cache/nginx /tmp /etc/nginx

COPY --from=build /app/build /var/www
COPY ./nginx.conf /etc/nginx/nginx.conf

USER frontend
EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
