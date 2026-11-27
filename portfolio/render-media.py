import array
import math
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import wave

ffmpeg, recording = sys.argv[1:3]
probe = subprocess.run([ffmpeg, '-hide_banner', '-i', recording], capture_output=True, text=True)
hours, minutes, seconds = map(float, re.search(r'Duration: (\d+):(\d+):([\d.]+)', probe.stderr).groups())
duration = hours * 3600 + minutes * 60 + seconds
assert math.isfinite(duration) and 0 < duration < 180
output = Path(__file__).parent / 'showcases/invar-v1.mp4'
with tempfile.TemporaryDirectory() as directory:
    score = Path(directory) / 'original-ambient.wav'
    rate = 48000
    with wave.open(str(score), 'wb') as audio:
        audio.setparams((1, 2, rate, 0, 'NONE', 'not compressed'))
        for start in range(0, int(duration * rate), 4096):
            samples = array.array('h')
            for index in range(start, min(start + 4096, int(duration * rate))):
                t = index / rate
                fade = max(0, min(1, t / 1.5, (duration - t) / 2))
                signal = sum(gain * math.sin(2 * math.pi * frequency * t) for frequency, gain in [(110, 1), (165, .45), (220, .2), (330, .08)])
                samples.append(int(32767 * .06 * fade * (.9 + .1 * math.sin(t * 1.2)) * signal))
            if sys.byteorder != 'little':
                samples.byteswap()
            audio.writeframes(samples.tobytes())
    subprocess.run([
        ffmpeg, '-hide_banner', '-loglevel', 'warning', '-y', '-i', recording, '-i', str(score),
        '-vf', 'scale=1280:720,fps=24', '-c:v', 'libx264', '-preset', 'medium', '-crf', '27',
        '-pix_fmt', 'yuv420p', '-c:a', 'aac', '-b:a', '96k', '-ac', '2',
        '-shortest', '-movflags', '+faststart', '-metadata', 'title=Invar — Change code. Preserve behavior.',
        '-metadata', 'comment=Actual website recording; illustrative comparison. Original synthesized ambient score.',
        str(output),
    ], check=True)
print(f'{output}: {output.stat().st_size} bytes; {duration:.2f} seconds')
