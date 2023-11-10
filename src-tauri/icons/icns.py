# https://pypi.org/project/icnsutil/
import icnsutil
# compose
img = icnsutil.IcnsFile()

# MAC OS 上执行？
img.add_media(file='512x512.png')
img.write('./icon.icns')