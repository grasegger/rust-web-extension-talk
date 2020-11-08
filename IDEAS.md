# These are some things you can try out before trying to write a full blown extension

## 1) Install Date Notification

You can replace the alert in the show_install_date_on_github script to display a real notification. Have a look at https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/user_interface/Notifications for that.

## 2) Communicate with the background

Try implementing a script, that talks from a website to the background. You can read about the concept on https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Content_scripts#Communicating_with_background_scripts. 

A sample use case:

Instead of letting each tab communicate with an api you can let the background keep the connection and cache answers there, so when you have a hundred tabs needing the data you don't make a hundred requests.

## 3) Create a popup

Many extensions have a popup in the toolbar of the browser. You can read up on how to implement one on https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/user_interface/Popups. 

## 4) Create an options page

Similiar to popups you can create a page to display options for your extensions. Read more on https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/user_interface/Options_pages. 