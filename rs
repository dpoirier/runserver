#!/bin/sh

if [ "$1" = . ] ; then
   # don't start browser
   start_browser=no
   shift
fi

case $VIRTUAL_ENV in
  */Project1) PORT=8001;;
  */Project2) PORT=8002;;
  # ...
  *) PORT=8000;;
esac
TARGET=0.0.0.0:$PORT
if [ "$start_browser" != "no" ] ; then
  (sleep 3;chromium-browser --temp-profile http://$TARGET )&
fi
exec python manage.py runserver "$@" "$TARGET"
