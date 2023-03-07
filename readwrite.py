import time
outfile = open("/tmp/outpy", "w")

while True:
    with open("/dev/random", "r") as infile:
        outfile.write("Test")
        time.sleep(1)
