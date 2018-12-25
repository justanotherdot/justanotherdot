FROM debian:testing

#ARG AWS_ACCESS_KEY_ID
#ARG AWS_SECRET_ACCESS_KEY
#ENV AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
#ENV AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY

COPY ./ ./blog
WORKDIR ./blog
RUN pip install awscli --upgrade --user && aws --version
      #build-site


