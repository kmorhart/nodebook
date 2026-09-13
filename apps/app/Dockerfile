FROM node:22-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:22-alpine
WORKDIR /app

RUN npm install -g sirv-cli

COPY --from=builder /app/dist ./dist

EXPOSE 7004

CMD ["sirv", "dist", "--port", "7004", "--host", "0.0.0.0", "--single"]