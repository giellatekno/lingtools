FROM node:22-bookworm

#RUN apt-get update && apt-get install curl
#RUN curl -fsSL https://get.pnpm.io/install.sh | SHELL=/bin/sh sh -

RUN mkdir /app
WORKDIR /app
COPY package.json /app
RUN npm install

COPY build /app

CMD [ "node", "index.js" ]
