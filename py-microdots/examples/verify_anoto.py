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

    # Generate 1 random shape and section
    SHAPES_ARR = [(random.randint(10, 70), random.randint(20, 70)) for _ in range(1)]
    SECTIONS_ARR = [(random.randint(5, 70), random.randint(5, 70)) for _ in range(1)]

    ### 1
    SHAPES_ARR.append((9, 16))
    SECTIONS_ARR.append((10, 2))

    SHAPES_ARR.append((9, 16))
    SECTIONS_ARR.append((10, 10))
    ### 2
    SHAPES_ARR.append((20, 25))
    SECTIONS_ARR.append((10, 2))

    SHAPES_ARR.append((20, 25))
    SECTIONS_ARR.append((10, 10))
    ### 3
    SHAPES_ARR.append((20, 20))
    SECTIONS_ARR.append((10, 2))

    SHAPES_ARR.append((20, 20))
    SECTIONS_ARR.append((10, 10))
    ### 4
    SHAPES_ARR.append((25, 20))
    SECTIONS_ARR.append((10, 2))

    SHAPES_ARR.append((25, 20))
    SECTIONS_ARR.append((10, 10))


    # Delete and create output directory
    if os.path.exists('output'):
        shutil.rmtree('output')
    os.makedirs('output')

    # Render dots
    import matplotlib.pyplot as plt

    
    for dot_shape, dot_section in zip(SHAPES_ARR, SECTIONS_ARR):
        G = codec.encode_bitmatrix(shape=dot_shape, section=dot_section)

        print(f"G shape: {dot_shape}, section: {dot_section}")
        # POS (3,7) grab 6x6
        S = G[3 : 3 + 6, 7 : 7 + 6]

        pos = codec.decode_position(S)
        sec = codec.decode_section(S, pos=pos)

        x_size = dot_shape[0]
        y_size = dot_shape[1]
        x_sect = dot_section[0]
        y_sect = dot_section[1]

        filename_json = f'output/PY__{x_size}_{y_size}__{x_sect}_{y_sect}.json'
        filename_txt = f'output/PY__{x_size}_{y_size}__{x_sect}_{y_sect}.txt'

        with open(filename_json, 'w') as f:
            json.dump(G.tolist(), f)

        with open(filename_txt, 'w') as f:
            f.write(f"POS: {pos}\n")
            f.write(f"SEC: {sec}\n")
            for row in G:
                f.write(' '.join(map(str, row)) + '\n')

        # XXXX

        filename_X_pdf = f'output/PY__{x_size}_{y_size}__{x_sect}_{y_sect}__X.pdf'

        fig, ax = plt.subplots(figsize=(x_size, y_size))
        mdots.draw_dots(G, grid_size=1.0, show_grid=True, ax=ax, dot_scale=400.0)
        ax.tick_params(axis='both', labelsize=25)

        ax.set(xlim=(-1, y_size), ylim=(-1, x_size))
        # ax.set_xlim(left=-1)
        # ax.set_ylim(bottom=-1)

        fig.savefig(filename_X_pdf)
        plt.close(fig)

        # YYYY
        filename_Y_pdf = f'output/PY__{x_size}_{y_size}__{x_sect}_{y_sect}__Y.pdf'

        fig, ax = plt.subplots(figsize=(x_size, y_size))
        mdots.draw_dots(G, grid_size=1.0, show_grid=True, ax=ax, dot_scale=400.0)
        ax.tick_params(axis='both', labelsize=25)

        # ax.set(xlim=(-1, x_size), ylim=(-1, y_size))
        # ax.set_xlim(left=-1)
        # ax.set_ylim(bottom=-1)
        fig.savefig(filename_Y_pdf)
        plt.close(fig)

main()