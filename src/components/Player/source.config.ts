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
  // 跨域资源
  {
    label: '中国之声-电台[M3U8]',
    value: 'https://ngcdn001.cnr.cn/live/zgzs/index.m3u8',
  },
  {
    label: '海洋世界[MP4]',
    value: 'http://vjs.zencdn.net/v/oceans.mp4',
  },
];

export default list;
