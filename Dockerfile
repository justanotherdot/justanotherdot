FROM debian:testing

ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY
ENV AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
ENV AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY

COPY ./ ./blog
WORKDIR ./blog
RUN apt -y update && apt -y install s4cmd && \
      s4cmd --access-key="${AWS_ACCESS_KEY_ID}"\
            --secret-key="${AWS_SECRET_ACCESS_KEY}"\
            get s3://justanotherdot.nyc3.digitaloceanspaces.com/build-site # && \
      ls # && \
      #build-site


