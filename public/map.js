mapboxgl.accessToken = 'pk.eyJ1Ijoid2FybXN0IiwiYSI6ImNsOWJydXhzNjNyN2EzdmxmZW00bDN0OTEifQ.4pmON-6Tdj9ca_T8_8D_dw';
var map = new mapboxgl.Map({
    container: 'map',
    style: 'mapbox://styles/mapbox/light-v10'
});

var canReport = false;
var isRegistering = false;

var lat = 0;
var long = 0;

map.on('click', (e) => {
    if (canReport || isRegistering) {
        let old_marker = document.getElementById("user marker");

        if (old_marker != null) {
            old_marker.remove();
        }

        var coords = `lat: ${e.lngLat.lat} <br> lng: ${e.lngLat.lng}`;

        // create the popup
        var popup = new mapboxgl.Popup().setText(coords);

        // create DOM element for the marker
        var el = document.createElement('div');
        el.id = 'user marker';
        el.classList.add("orange");

        // create the marker
        new mapboxgl.Marker(el)
            .setLngLat(e.lngLat)
            .setPopup(popup)
            .addTo(map);   
    
        lat = e.lngLat.lat;
        long = e.lngLat.lng;
    }
    
    if (canReport) {
        displayForm();
    }

    else if (isRegistering) {
        register_user(parseInt(document.getElementById("phone_num").value), lat, long);
        isRegistering = false;

        let old_marker = document.getElementById("user marker");

        if (old_marker != null) {
            old_marker.remove();
        }

        document.getElementById("prompt").style.display = "none";

    }

  });
