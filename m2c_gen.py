import sys
import m2cgen as m2c
import joblib
sys.setrecursionlimit(10000)
rf = joblib.load("../pangoLEARN/pangoLEARN/data/decisionTree_v1.joblib")
rf_rs = m2c.export_to_rust(rf)
f = open("pangolearn-classifier/src/score.rs", 'w')

f.write("//this file is machine generated! \n")
f.write("pub ")
f.write(rf_rs)
f.close()