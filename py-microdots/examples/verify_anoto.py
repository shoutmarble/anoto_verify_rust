
def main():
    #
    import json
    import random
    import os
    import shutil
    # Import the library
    import microdots as mdots

    # Use the default embodiment with A4 sequence fixed
    codec = mdots.anoto_6x6_a4_fixed

    # Generate random bit-matrix to verify with RUST version
    SHAPES_ARR = [(random.randint(10, 70), random.randint(20, 70)) for _ in range(10)]
    SHAPES_ARR.append((9, 16))
    SECTIONS_ARR = [(random.randint(5, 70), random.randint(5, 70)) for _ in range(10)]
    SECTIONS_ARR.append((10, 2))

    # Delete and create output directory
    if os.path.exists('output'):
        shutil.rmtree('output')
    os.makedirs('output')

    # Render dots
    import matplotlib.pyplot as plt

    
    for dot_shape, dot_section in zip(SHAPES_ARR, SECTIONS_ARR):
        G = codec.encode_bitmatrix(shape=(dot_shape), section=dot_section)

        print(f"G shape: {dot_shape}, section: {dot_section}")
        # Decode a partial matrix
        S = G[3 : 3 + 6, 7 : 7 + 6]

        pos = codec.decode_position(S)
        sec = codec.decode_section(S, pos=pos)

        filename_json = f'output/PY__{dot_shape[0]}_{dot_shape[1]}__{dot_section[0]}_{dot_section[1]}.json'
        filename_txt = f'output/PY__{dot_shape[0]}_{dot_shape[1]}__{dot_section[0]}_{dot_section[1]}.txt'

        with open(filename_json, 'w') as f:
            json.dump(G.tolist(), f)

        with open(filename_txt, 'w') as f:
            f.write(f"POS: {pos}\n")
            f.write(f"SEC: {sec}\n")
            for row in G:
                f.write(' '.join(map(str, row)) + '\n')

        filename_pdf = f'output/PY__{dot_shape[0]}_{dot_shape[1]}__{dot_section[0]}_{dot_section[1]}__X.pdf'

        fig, ax = plt.subplots(figsize=(20, 20))
        mdots.draw_dots(G, grid_size=1.0, show_grid=True, ax=ax)
        # Align Y-axis to start at 0 like X-axis and extend to show all data
        ax.set_ylim(-1, G.shape[0] )  # Set Y-axis from 0 to number of rows 
        ax.set_xlim(-1, G.shape[1] )  # Set X-axis from 0 to number of columns 
        fig.savefig(filename_pdf)
        plt.close(fig)

        filename_pdf = f'output/PY__{dot_shape[0]}_{dot_shape[1]}__{dot_section[0]}_{dot_section[1]}__Y.pdf'

        fig, ax = plt.subplots(figsize=(20, 20))
        mdots.draw_dots(G, grid_size=1.0, show_grid=True, ax=ax)
        # Align Y-axis to start at 0 like X-axis and extend to show all data
        ax.set_ylim(G.shape[0], -1)  # Set Y-axis from 0 to number of rows 
        ax.set_xlim(-1, G.shape[1] )  # Set X-axis from 0 to number of columns 
        fig.savefig(filename_pdf)
        plt.close(fig)



    # Generate a bit-matrix for section (10,2)
    G = codec.encode_bitmatrix(shape=(9, 16), section=(10, 2))

    # Render dots
    import matplotlib.pyplot as plt

    fig, ax = plt.subplots(figsize=(20, 20))
    mdots.draw_dots(G, grid_size=1.0, show_grid=True, ax=ax)
    # Align Y-axis to start at 0 like X-axis and extend to show all data
    ax.set_ylim(G.shape[0] + 1, -1)  # Set Y-axis from 0 to number of rows + 1
    ax.set_xlim(-1, G.shape[1] + 1)  # Set X-axis from 0 to number of columns + 1
    fig.savefig("dots.pdf")
    plt.close(fig)

    # Decode a partial matrix
    S = G[3 : 3 + 6, 7 : 7 + 6]

    pos = codec.decode_position(S)
    sec = codec.decode_section(S, pos=pos)

    # To decode the rotation, use an extended matrix
    R = G[3 : 3 + 8, 7 : 7 + 8]
    rot = codec.decode_rotation(R)
    print("pos:", pos, "sec:", sec, "rot:", rot)
    # > pos: (7, 3) sec: (10, 2) rot: 0


main()
