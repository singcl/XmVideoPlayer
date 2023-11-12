import { xmCl } from '@/internal/http/client';

/**
 * 提交反馈
 */
export async function feedbackUpdate(data: { message: string; email: string }) {
  return await (await xmCl).post<{ id: string }>('api/xmvideo/feedback/update', data);
}
