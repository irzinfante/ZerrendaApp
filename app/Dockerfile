FROM node:18-alpine AS build
WORKDIR /app
COPY package*.json tsconfig*.json vite.config.ts index.html .env ./
COPY src ./src
COPY public ./public
RUN npm ci
RUN npm run build

FROM nginx:stable-alpine AS runtime
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]