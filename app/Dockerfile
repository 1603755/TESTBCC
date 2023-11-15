FROM node

RUN mkdir -p /srv/app/app
WORKDIR /srv/app/app

COPY package.json /srv/app/app


RUN npm install

COPY . /srv/app/app
EXPOSE 4321
CMD ["npm", "run", "start", "--host" ]