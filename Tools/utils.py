import geopandas
from graphviz import Source
from shapely.geometry import Point, LineString
from shapely.wkt import loads

def read_graph_dot():
    with open('../data/graph.dot', 'r') as file:
        lines = file.readlines()

    points_wkt = []
    for line in lines:
        if line.find("Point") == -1:
            continue
        else:
            start = line.find("{")
            end = line.find("}", start+1)
            wkt_stlye = "POINT(" + line[start+5:end].replace(", y: ", " ") + ")"
            point = loads(wkt_stlye)
            points_wkt.append(point)

    geom = []
    for i,p in enumerate(points_wkt):
        try: 
            geom.append(LineString([p,points_wkt[i+1]]))
        except IndexError:
            pass
    points_wkt = points_wkt + geom

    ids = [0] * len(points_wkt)
    highway = ["None"] * len(points_wkt)
    data = {
        "osm_id": ids,
        "highway": highway,
        "geometry": points_wkt,
    }


    graph = geopandas.GeoDataFrame(data, crs="EPSG:4326")
    return graph

def read_solution_path_txt():
    with open('../data/solution_path.txt', 'r') as file:
        lines = file.read().split("Point")

    points_wkt = []
    for line in lines:
        if line.find("{") == -1:
            continue
        else:
            start = line.find("{")
            end = line.find("}", start+1)
            wkt_stlye = "POINT(" + line[start+5:end].replace(", y: ", " ") + ")"
            point = loads(wkt_stlye)
            points_wkt.append(point)

    geom = []
    for i,p in enumerate(points_wkt):
        try: 
            geom.append(LineString([p,points_wkt[i+1]]))
        except IndexError:
            pass
    points_wkt = points_wkt + geom

    ids = [0] * len(points_wkt)
    highway = ["None"] * len(points_wkt)
    data = {
        "osm_id": ids,
        "highway": highway,
        "geometry": points_wkt,
    }
    solution = geopandas.GeoDataFrame(data, crs="EPSG:4326")
    return solution