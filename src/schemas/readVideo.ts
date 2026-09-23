import { z } from 'zod';

export const videoSourceSchema = z.object({
  path: z.string().min(1).describe('Path to the local video file (absolute or relative to cwd).'),
});

export const readVideoArgsSchema = z.object({
  sources: z.array(videoSourceSchema).min(1).describe('One or more local video sources to read.'),
  profile: z
    .enum(['fast', 'quality'])
    .optional()
    .describe(
      'Predictable work profile. fast returns container metadata, streams, chapters, and embedded subtitles. quality adds ffmpeg scene detection. Neither profile extracts frames or runs speech recognition.'
    ),
  include_streams: z
    .boolean()
    .optional()
    .describe('Include stream metadata from ffprobe. Defaults to true.'),
  include_chapters: z
    .boolean()
    .optional()
    .describe('Include chapter markers when present. Defaults to true.'),
  include_subtitles: z
    .boolean()
    .optional()
    .describe('Extract embedded subtitles when available. Defaults to true.'),
  include_scenes: z
    .boolean()
    .optional()
    .describe(
      'Detect scene boundaries with ffmpeg scene filter. Off unless profile is quality or this flag is set.'
    ),
  scene_threshold: z
    .number()
    .min(0)
    .max(1)
    .optional()
    .describe('Scene detection sensitivity for ffmpeg gt(scene,threshold). Defaults to 0.4.'),
  include_transcript: z
    .boolean()
    .optional()
    .describe(
      'Request speech recognition. Defaults to false, including on quality. The shipped Rust server does not run it and returns a warning.'
    ),
  include_keyframes: z
    .boolean()
    .optional()
    .describe(
      'Request keyframe locators. Defaults to false, including on quality. The shipped Rust server does not extract them and returns a warning. A frame is video_evidence.'
    ),
  keyframe_limit: z
    .number()
    .int()
    .min(1)
    .max(64)
    .optional()
    .describe(
      'Maximum number of keyframe locators to return when include_keyframes is true. Defaults to 8.'
    ),
  include_keyframe_images: z
    .boolean()
    .optional()
    .describe(
      'When include_keyframes is true, render citeable PNG thumbnails for each keyframe. Defaults to false.'
    ),
  keyframe_max_dimension: z
    .number()
    .int()
    .positive()
    .optional()
    .describe('Maximum width or height when resizing keyframe PNG evidence.'),
  keyframe_policy: z
    .enum(['structural', 'iframes'])
    .optional()
    .describe(
      'structural (default when include_keyframes): scene starts/midpoints. iframes: raw I-frame index only.'
    ),
  include_agent_index: z
    .boolean()
    .optional()
    .describe(
      'Return agent_index outline so text-only agents can read film structure. Defaults to true.'
    ),
});

export type ReadVideoArgs = z.infer<typeof readVideoArgsSchema>;
export type VideoSource = z.infer<typeof videoSourceSchema>;
