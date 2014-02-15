import struct

MAGIC = ['M', 'A', 'P']
VERSION = 1
tile_dict = { '.' : 0, '#' : 1 }

in_file = file('../assets/maps/map1.txt')
translated = []

width = None
height = 0

for line in in_file:
    width = len(line.rstrip('\n'))
    height += 1
    for char in line.rstrip('\n'):
        translated.append(tile_dict[char])
            
in_file.close()
trans_len = len(translated)

out_file = open('../assets/maps/map1', 'wb')
out_file.write(struct.pack('ccc', *MAGIC))
out_file.write(struct.pack('B', VERSION))
out_file.write(struct.pack('I', width))
out_file.write(struct.pack('I', height))
out_file.write(struct.pack(trans_len*'H', *translated))
out_file.close()
