<script type="application/json" def>
{
  "navigationBarTitleText": "照片浮窗",
  "description": "Displays a locally bundled photo or animated GIF on Rokid glasses using configurable position, size, opacity, and visibility settings.",
  "schema": {
    "data": {
      "type": "object",
      "properties": {}
    }
  }
}
</script>

<script setup>
export default {
  data: {
    imageSrc: '/assets/portrait_default.png',
    anchorClass: 'right_middle',
    sizeClass: 'small',
    opacityClass: 'opacity_60',
    visibilityClass: 'shown',
  },
};
</script>

<page>
  <view class="screen">
    <image
      class="portrait {{ anchorClass }} {{ sizeClass }} {{ opacityClass }} {{ visibilityClass }}"
      src="{{ imageSrc }}"
      mode="widthFix"
    ></image>
  </view>
</page>

<style>
page,
.screen {
  width: 448px;
  height: 352px;
  background-color: var(--color-background);
  overflow: hidden;
}

.screen {
  position: relative;
}

.portrait {
  position: absolute;
}

.small { width: 96px; }
.medium { width: 140px; }
.large { width: 190px; }

.left_top { left: 18px; top: 18px; }
.right_top { right: 18px; top: 18px; }
.left_middle { left: 18px; top: 50%; transform: translateY(-50%); }
.right_middle { right: 18px; top: 50%; transform: translateY(-50%); }
.left_bottom { left: 18px; bottom: 18px; }
.right_bottom { right: 18px; bottom: 18px; }

.opacity_40 { opacity: 0.4; }
.opacity_60 { opacity: 0.6; }
.opacity_80 { opacity: 0.8; }
.opacity_100 { opacity: 1; }
.shown { visibility: visible; }
.hidden { visibility: hidden; }
</style>
