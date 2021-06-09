<template>
  <canvas :id="name" :width="width" :height="height" class="box-card-canvas" />
</template>

<script>
export default {
  props: ["name", "width", "height", "min", "max", "value", "inverse", "color"],

  data() {
    return {
      context: null,
    };
  },

  mounted() {
    const canvas = document.getElementById(this.name);
    this.context = canvas.getContext("2d");
    this.draw();
  },

  computed: {
    normValue: function () {
      return (this.value - this.min) / (this.max - this.min);
    },
  },

  watch: {
    value: function () {
      this.draw();
    },
  },

  methods: {
    draw: function () {
      this.context.clearRect(0, 0, this.width, this.height);
      this.context.fillStyle = this.color;
      if (this.inverse) {
        this.context.fillRect(
          0,
          0,
          this.width,
          (1 - this.normValue) * this.height
        );
      } else {
        this.context.fillRect(
          0,
          this.height,
          this.width,
          -this.normValue * this.height
        );
      }
    },
  },
};
</script>

<style>
.box-card-canvas {
  background-color: #ffffff;
  padding: 1px;
}
</style>