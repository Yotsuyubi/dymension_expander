<template>
  <el-card class="box-card">
    <span class="slider-label"> {{ name }} [{{ unit }}]</span>
    <el-slider
      v-model="value"
      :max="max"
      :min="min"
      :step="step"
      :width="width"
      :height="height"
      :ref="name"
      class="slider"
    >
    </el-slider>
  </el-card>
</template>

<script>
export default {
  props: ["name", "width", "height", "min", "max", "step", "unit", "value"],
  mounted() {
    const controller = this.$refs[`${this.name}`];
    controller.addEventListener("onmousedown", (e) => {
      external.invoke(`edit-begin-${this.name}`);
    });
    controller.addEventListener("onmouseup", (e) => {
      external.invoke(`edit-end-${this.name}`);
    });
  },
  watch: {
    value: function () {
      this.$emit(`${this.name}`, this.value);
      external.invoke(`set-${this.name} ${this.value}`);
    },
  },
};
</script>

<style>
.box-card {
  background-color: #ffffff;
  margin: 5px;
}

.slider-label {
  font-size: 1em;
  color: #131212;
  font-family: "Baloo Tammudu 2", cursive;
}
</style>