import fileinput
import re
import os
import numpy as np
import matplotlib.pyplot as plt
import logging
import argparse

logging.basicConfig(filename='base.log', level=logging.DEBUG)

trace = "unknown"
data_structure="unknown"
operation = "unknown"
length = 1
results = {}

parser = argparse.ArgumentParser()
parser.add_argument('-r', '--restrict', action='store', default='BASIC')
parser.add_argument('-p', '--path', action='store', default='..\\..\\rust-projects\\filters_and_sketches\\results')

restricts = {}
restricts['BASIC'] = ['SpaceSaving', 'SpaceSaving-RAP', 'CMS', 'NitroCMS', 'HASH', 'NitroHash', 'Cuckoo', 'NitroCuckoo']
restricts['OPTS-FULL'] = ['CMS', 'NitroCMS', 'CMS-NOMI', 'Cuckoo', 'NitroCuckoo', 'NitroCuckoo-SMALL']
restricts['OPTS'] = ['CMS', 'CMS-NOMI', 'NitroCuckoo', 'NitroCuckoo-SMALL']
restricts['NOMI'] = ['CMS', 'CMS-NOMI']
restricts['NITRO'] = ['Cuckoo', 'NitroCuckoo', 'NitroCuckoo-SMALL']
args = parser.parse_args()
if not(args.restrict in restricts.keys()):
    print('Found an unknown restrict parameter ' + args.restrict + ' - using all algorithms.')

trace_text = re.compile('TRACE .*')
dstype_text = re.compile('DSTYPE .*')
test_text = re.compile('TEST .*')
length_text = re.compile('LENGTH .*')
oamsre_text = re.compile('On-Arrival MSRE .*')
oaavgerr_text = re.compile('On-Arrival AVGERR .*')
oaavgrelerr_text = re.compile('On-Arrival AVGRELERR .*')
flowmsre_text = re.compile('Flow MSRE .*')
flowavgerr_text = re.compile('Flow AVGERR .*')
flowavgrelerr_text = re.compile('Flow AVGRELERR .*')
pmwmsre_text = re.compile('PMW MSRE .*')
pmwavgerr_text = re.compile('PMW AVGERR .*')
pmwavgrelerr_text = re.compile('PMW AVGRELERR .*')
totalmemory_text = re.compile('Total memory: .*')
items_text = re.compile('Number of items: .*')
time_text = re.compile('TIMEms .*')
end_text = re.compile('END .*')

shortened = {}
shortened["SpaceSaving"] = "SS"
shortened["SpaceSaving-RAP"] = "SS-RAP"
shortened["NitroCuckoo-SMALL"] = "NC-SMALL"

algcolors={}
algcolors["DEFAULT"] = 'cyan'
algcolors["SpaceSaving"] = 'black'
algcolors["SpaceSaving-RAP"] = 'red'
algcolors["CMS"] = 'green'
algcolors["NitroCMS"] = 'blue'
algcolors["HASH"] = 'orange'
algcolors["NitroHash"] = 'purple'
algcolors["Cuckoo"] = 'grey'
algcolors["NitroCuckoo"] = 'pink'
algcolors["NitroCuckoo-SMALL"] = 'olive'
algcolors["CMS-NOMI"] = 'brown'

altylegend={}
altylegend["SPACE"]="Space (Bytes)"
altylegend["MEMORY"]="Memory (Bytes)"
altylegend["ITEMS"]="Items"
altylegend["Throughput"] = "Throughput (items/sec)"


def generic_parser(line):
    logging.debug("Generic: %s", line)


def trace_parser(line):
    global trace
    trace = os.path.basename(line).split('.')[0]
    logging.debug("Trace: %s", trace)
    if trace not in results:
        results[trace] = {}


def test_parser(line):
    global operation
    operation = line.split().pop()
    logging.debug("Test: %s", operation)
    if operation not in results[trace]:
        results[trace][operation] = {}


def dstype_parser(line):
    global data_structure
    data_structure = line.split().pop()
    logging.debug("DataStructure: %s", data_structure)
    if data_structure not in results[trace][operation]:
        results[trace][operation][data_structure] = {}


def length_parser(line):
    global length
    length = line.split().pop()
    logging.debug("Length: %s", length)


def generic_result_parser(line,res_name):
    res = line.split().pop()
    logging.debug("%s:%s",res_name,res)
    if res_name not in results[trace][operation][data_structure]:
        results[trace][operation][data_structure][res_name] = [res]
    else:
        results[trace][operation][data_structure][res_name].append(res)
    logging.debug("%s values: %s", res_name, results[trace][operation][data_structure][res_name])


def oamsre_parser(line):
    generic_result_parser(line,"OA-MSRE")


def oaavgerr_parser(line):
    generic_result_parser(line, "OA-AVGERR")


def oaavgrelerr_parser(line):
    generic_result_parser(line, "OA-AVGRELERR")


def flowmsre_parser(line):
    generic_result_parser(line, "FLOW-MSRE")


def flowavgerr_parser(line):
    generic_result_parser(line, "FLOW-AVGERR")


def flowavgrelerr_parser(line):
    generic_result_parser(line, "FLOW-AVGRELERR")


def pmwmsre_parser(line):
    generic_result_parser(line, "PMW-MSRE")


def pmwavgerr_parser(line):
    generic_result_parser(line, "PMW-AVGERR")


def pmwavgrelerr_parser(line):
    generic_result_parser(line, "PMW-AVGRELERR")


def totalmemory_parser(line):
    generic_result_parser(line, "MEMORY")


def time_parser(line):
    generic_result_parser(line, "TIME(ms)")


def items_parser(line):
    parts = line.split()
    items = parts[3]
    logging.debug("ITEMS: %s", items)
    if "ITEMS" not in results[trace][operation][data_structure]:
        results[trace][operation][data_structure]["ITEMS"] = [items]
    else:
        results[trace][operation][data_structure]["ITEMS"].append(items)
    logging.debug("ITEMS values: %s", str(results[trace][operation][data_structure]["ITEMS"]))
    space = parts[5]
    logging.debug("SPACE: %s", space)
    if "SPACE" not in results[trace][operation][data_structure]:
        results[trace][operation][data_structure]["SPACE"] = [space]
    else:
        results[trace][operation][data_structure]["SPACE"].append(space)
    logging.debug("SPACE values: %s", str(results[trace][operation][data_structure]["SPACE"]))


my_parsers ={
    trace_text: trace_parser,
    dstype_text: dstype_parser,
    test_text: test_parser,
    length_text: length_parser,
    oamsre_text: oamsre_parser,
    oaavgerr_text: oaavgerr_parser,
    oaavgrelerr_text: oaavgrelerr_parser,
    flowmsre_text: flowmsre_parser,
    flowavgerr_text: flowavgerr_parser,
    flowavgrelerr_text: flowavgrelerr_parser,
    pmwmsre_text: pmwmsre_parser,
    pmwavgerr_text: pmwavgerr_parser,
    pmwavgrelerr_text: pmwavgrelerr_parser,
    totalmemory_text: totalmemory_parser,
    items_text: items_parser,
    time_text: time_parser
}

def process(line):
    for reg_expr, parse_func in my_parsers.items():
        if reg_expr.match(line) is not None:
            parse_func(line)
            return
    logging.debug("NOP: %s", line)


#respath = 'C:\\Users\\user\\rust-projects\\filters_and_sketches\\results'
#resfiles = [os.path.join(respath,file) for file in os.listdir(respath)]
resfiles = map(lambda x: x.path,filter(lambda y: y.is_file(),os.scandir(args.path)))
print(resfiles)
for line in fileinput.input(resfiles):
    try:
        process(line)
    except:
        print("Process error: ", line)

fileinput.close()


## Press the green button in the gutter to run the script.
#if __name__ == '__main__':
#    print_hi('PyCharm')

def get_short_name(key):
    if key in shortened:
        return shortened[key]
    else:
        return key


def get_alg_color(key):
    if key in algcolors:
        return algcolors[key]
    else:
        return key


file_prefix = ""
if args.restrict != 'BASIC':
    file_prefix = args.restrict + "-"


def generate_graph(trace,op,met,vals):
    averaged = list(map(lambda t: t[0], vals.values()))
    stdeved = list(map(lambda t: t[1], vals.values()))
    named = list(map(get_short_name,vals.keys()))
    colorbars = list(map(get_alg_color,vals.keys()))
    plt.figure(figsize=(1.8 * len(vals.keys()), 6))
    plt.rc('xtick', labelsize=18)
    plt.rc('ytick', labelsize=18)
    plt.bar(named,averaged,color=colorbars)
    plt.errorbar(named, averaged, yerr=stdeved, fmt="o", color="r",capsize=10)
    print(trace + "-" + met + " avgs:" + " ".join(map(str,averaged)) + " errs:" + " ".join(map(str,stdeved)))
    plt.xlabel('Algorithm',fontsize=20)
    if met in altylegend:
        plt.ylabel(altylegend[met], fontsize=20)
    else:
        plt.ylabel(met,fontsize=20)
    if met == "Throughput":  # sorry for the hack
        plt.savefig(file_prefix + trace + "-" + met + "-" + op + ".png", bbox_inches='tight')
    else:
        plt.savefig(file_prefix + trace+"-"+met+".png", bbox_inches='tight')
    plt.close()
    #fig.write_image(trace+"-"+met+".pdf")
    #fig.write_html(trace + "-" + met + ".html")


def sort_bars(bars_dict):
    bar_keys = sorted(bars_dict.keys())
    return {i: bars_dict[i] for i in bar_keys}


for trace,operations in results.items():
    for op,datastructures in operations.items():
        mets = {}
        for ds,metrics in datastructures.items():
            if (not(args.restrict in restricts.keys())) | (ds in restricts[args.restrict]):
                for met,values in metrics.items():
                    floated = list(map(float,values))
                    if met == "TIME(ms)":  # sorry for the hack
                        floated = list(map(lambda t: 1000000.0*(float(length))/t,floated))
                        met = "Throughput"
                    averaged = sum(floated) / len(floated)
                    stdeved = np.std(floated)
                    if met not in mets.keys():
                        mets[met] = {}
                    mets[met][ds]=(averaged,stdeved)
        for met,vals in mets.items():
            generate_graph(trace,op,met,sort_bars(vals))
            #print(trace+':'+op+':'+ds+':'+met+':',sum(list(map(float,values))))

