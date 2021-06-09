import Vue from 'vue'
import App from './App'
import Element from 'element-ui'

Vue.use(Element)

new Vue({
    el: '#app',
    components: { App },
    template: '<app/>',
})