# Vigilant
A comunity driven crime reporting app created as a part of DubHacks 2022.

![demo gif](https://github.com/explosion33/Vigilant/blob/main/demo.gif)
<img src="https://github.com/explosion33/Vigilant/blob/main/sms.png" alt="sms demo" width="400"/>

## About
Vigilant is a web app where users can report and get notified to crimes in their local communities. Vigilant fills a void in community crime awareness, by allowing for the immediate reporting of crimes. Users can register to recieve SMS messages of crimes within a 6 mile radius of thier chosen location, report crimes they've witnessed and view a heatmap of crimes around their local area

## Tech
### Back-end
The back-end API is written in Rust using the Rocket / Tokio framework. The back-end manages API requests to MapBox, for displaying realtime maps and Geocoding locations, as well as Twilio, for sending user alerts.

### Front-end
The front end is written with HTML, CSS, and pure JavaScript.


## What's to come
This project was made in under 24 hours ... while we have a functional prototype there is still much more we would like to add.

In the future we plan on implemnting:
* More responsive UI elements
* Better user action confirmation
* Support for merging crimes / upvoting crimes
* User reputation ratings
* Crime history / analytics
* Route planning based on predicted crime
* Collaboration with local authorities
