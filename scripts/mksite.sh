stack exec build-site
#bulma_version="0.5.2"
#curl -o dist/assets/bulma.min.css https://cdnjs.cloudflare.com/ajax/libs/bulma/"$bulma_version"/css/bulma.min.css
mkdir dist/assets
cp assets/bulma.min.css dist/assets/.
cp -r dist/* deploy/.
