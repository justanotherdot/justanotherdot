FROM debian:testing
COPY ./ ./blog
WORKDIR ./blog
RUN apt -y update && apt -y install s4cmd && \
      s4cmd --access-key="${AWS_ACCESS_KEY_ID}"\
            --secret-key="${AWS_SECRET_ACCESS_KEY}"\
            get s3://justanotherdot.nyc3.digitaloceanspaces.com/build-site # && \
      ls # && \
      #build-site


