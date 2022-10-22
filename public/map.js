mapboxgl.accessToken = 'pk.eyJ1Ijoid2FybXN0IiwiYSI6ImNsOWJydXhzNjNyN2EzdmxmZW00bDN0OTEifQ.4pmON-6Tdj9ca_T8_8D_dw';
var map = new mapboxgl.Map({
    container: 'map',
    style: 'mapbox://styles/mapbox/streets-v11'
});

map.on('click', (e) => {
    console.log(e.lngLat.lat);
    console.log(e.lngLat.lng);

    // create the popup
    var popup = new mapboxgl.Popup().setText(coords);

    // create DOM element for the marker
    var el = document.createElement('div');
    el.id = 'marker';

    // create the marker
    new mapboxgl.Marker(el)
        .setLngLat(e.lngLat)
        .setPopup(popup)
        .addTo(map);   
        
        
    // TODO SAVE DATA
  });
