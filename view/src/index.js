import Vue from 'vue'
import App from './App'
import { Slider, Card, Header, Container } from 'element-ui'

Vue.component(Slider.name, Slider)
Vue.component(Card.name, Card)
Vue.component(Header.name, Header)
Vue.component(Container.name, Container)

new Vue({
    el: '#app',
    components: { App },
    template: '<app/>',
})