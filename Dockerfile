FROM nginx:alpine

COPY ./out /usr/share/nginx/html

EXPOSE 80