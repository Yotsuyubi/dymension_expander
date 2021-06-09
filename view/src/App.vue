<template>
  <el-container class="body">
    <el-header class="header">
      <span class="header-title"> Dymension Expander </span>
    </el-header>
    <el-row :gutter="10">
      <el-col :span="20" class="controller">
        <Slider
          name="threshold"
          width="100px"
          height="200px"
          unit="dB"
          :min="-80.0"
          :max="0.0"
          :step="1.0"
          @gain="emitThreshold"
          v-model="th"
        />
        <Slider
          name="ratio"
          width="100px"
          height="200px"
          unit="-"
          :min="1"
          :max="30"
          :step="0.1"
          @gain="emitRatio"
          v-model="ratio"
        />
        <Slider
          name="attack"
          width="100px"
          height="200px"
          unit="ms"
          :min="1"
          :max="1e3"
          :step="1"
          @gain="emitAttack"
          v-model="attack"
        />
        <Slider
          name="release"
          width="100px"
          height="200px"
          unit="ms"
          :min="20"
          :max="3e3"
          :step="1"
          @gain="emitRelease"
          v-model="release"
        />
      </el-col>
      <el-col :span="4" class="controller">
        <Mater
          name="gainDynamics"
          :width="50"
          :height="410"
          :min="-80.0"
          :max="0.0"
          v-model="gain_dynamics"
          :inverse="false"
        />
        <Mater
          name="gainDynamics"
          :width="10"
          :height="410"
          :min="-80.0"
          :max="0.0"
          v-model="gain_dynamics"
          :inverse="true"
        />
      </el-col>
    </el-row>
  </el-container>
</template>

<script>
import Slider from "./components/Slider";
import Mater from "./components/Mater";

export default {
  components: {
    Slider,
    Mater,
  },
  data() {
    return {
      th: 0.0,
      ratio: 1.0,
      attack: 20.0,
      release: 100.0,
      gain_dynamics: 0.0,
    };
  },
  mounted() {
    this.update();
  },
  methods: {
    update: function () {
      const self = this;
      this.interval = setInterval(function () {
        self.getter();
      }, 1);
    },

    getter: function () {
      this.th = Number(external.invoke("get-threshold"));
      this.ratio = Number(external.invoke("get-ratio"));
      this.attack = Number(external.invoke("get-attack"));
      this.release = Number(external.invoke("get-release"));
      this.gain_dynamics = Number(external.invoke("get-gainDynamics"));
    },

    emitThreshold: function (value) {
      this.th = value;
    },
    emitRatio: function (value) {
      this.ratio = value;
    },
    emitAttack: function (value) {
      this.attack = value;
    },
    emitRelease: function (value) {
      this.release = value;
    },
  },
  watch: {},
};
</script>

<style>
.body {
  width: 480px;
  height: 480px;
  background-color: #ffffff;
}

.header-title {
  vertical-align: middle;
  display: inline-block;
  font-size: 32px;
  color: #131212;
  font-family: "Baloo Tammudu 2", cursive;
}

.el-card__body {
  padding: 18px 20px 5px 20px;
}

.el-col-20 {
  padding-left: 0px;
  padding-right: 0px;
}
</style>