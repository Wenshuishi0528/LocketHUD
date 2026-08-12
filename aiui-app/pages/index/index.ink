<script type="application/json" def>
{
  "navigationBarTitleText": "照片浮窗",
  "description": "Displays one locally precomposed 448 x 352 photo or animated GIF frame on Rokid glasses without additional layout or image scaling.",
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
    imageSrc: '/assets/display_default.png',
  },
};
</script>

<page>
  <view class="screen">
    <image class="frame" src="{{ imageSrc }}" mode="scaleToFill"></image>
  </view>
</page>

<style>
page,
.screen,
.frame {
  width: 448px;
  height: 352px;
}

page,
.screen {
  background-color: var(--color-background);
}
</style>
