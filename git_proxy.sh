#!/bin/bash
case $1 in
 
on)
git config --global http.proxy 'http://127.0.0.1:10809' 
git config --global https.proxy 'http://127.0.0.1:10809'
;;
 
off)
git config --global --unset http.proxy
git config --global --unset https.proxy
;;
 
status)
git config --get http.proxy
git config --get https.proxy
;;
esac
exit 0