FROM debian:testing
COPY ./ ./blog
WORKDIR ./blog
RUN apt -y update && apt -y install s4cmd && \
      #s4cmd --configure && \
      s4cmd get s3://justanotherdot.nyc3.digitaloceanspaces.com/build-site # && \
      ls # && \
      #build-site


