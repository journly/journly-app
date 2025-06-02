curl -s --user 'api:9c77d2e2f200e0f1729242c7c620411b-08c79601-12cbe009' \
  https://api.mailgun.net/v3/myjournly.com/messages \
  -F from='Journly Support <support@myjournly.com>' \
  -F to='Renchie Yang <renchieyang@gmail.com>' \
  -F subject='Hello Renchie' \
  -F template='journly community notification signup' \
  -F h:X-Mailgun-Variables='{"test": "test"}'

