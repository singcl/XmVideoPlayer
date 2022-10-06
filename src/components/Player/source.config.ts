import flour from './../../assets/flour.webp';
import redbag from './../../assets/redbag.webp';
import car from './../../assets/car.webp';
import xj from './../../assets/xj.webp';
import stars from './../../assets/stars.webp';
import plane1 from './../../assets/plane1.png';
import plane2 from './../../assets/plane2.png';

//
export const giftList = [
  {
    label: '鲜花',
    value: flour,
  },
  {
    label: '红包',
    value: redbag,
  },
  {
    label: '跑车',
    value: car,
  },
  {
    label: '仙境',
    value: xj,
  },
  {
    label: '大星星',
    value: stars,
  },
  {
    label: '飞机',
    value: plane1,
  },
  {
    label: '超级飞机',
    value: plane2,
  },
];

const list = [
  {
    label: '海洋世界[MP4]',
    value: 'http://vjs.zencdn.net/v/oceans.mp4',
  },
  {
    label: '毒液[MPEG-DASH]',
    value: 'https://storage.googleapis.com/wvmedia/clear/h264/tears/tears.mpd',
  },
  {
    label: '高山雪域[MP4]',
    value: 'https://media.w3.org/2010/05/sintel/trailer.mp4',
  },
  {
    label: '剑与魔法[M3U8]',
    value: 'https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8',
  },
  {
    label: '西瓜视频[FLV]',
    value: 'https://sf1-hscdn-tos.pstatp.com/obj/media-fe/xgplayer_doc_video/flv/xgplayer-demo-360p.flv',
  },
  {
    label: '小松鼠与熊[MPEG-DASH]',
    value: 'https://dash.akamaized.net/akamai/bbb_30fps/bbb_30fps.mpd',
  },
  {
    label: '监听事件测试[MPEG-DASH]',
    value: 'https://livesim.dashif.org/livesim/sts_1663490483/sid_a8436cd5/scte35_2/testpic_2s/Manifest.mpd',
  },
  // 跨域资源
  {
    label: '中国之声-电台[M3U8]',
    value: 'https://ngcdn001.cnr.cn/live/zgzs/index.m3u8',
  },
  {
    label: 'CCTV-6',
    value:
      'https://hlslive.1905.com/live/LIVE2OR14O20ADLYU/index.m3u8?tm=1665061052&sign=2f23852ed17350de63ce39d65a6d536c',
  },
];

export default list;
