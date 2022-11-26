from direct.actor.Actor import Actor
from direct.showbase.ShowBase import ShowBase  
from pandac import *
from direct import *
from panda3d.core import *
from direct.showbase.Loader import *

from . import rustycog

def gen_head(head:rustycog.Head):
    mod = loader.loadModel(head.file)
    mod.find("**/"+head.node)
    if head.texture:
        mod.setTexture(loader.loadTexture(head.texture),1)
    if head.color:
        mod.setColor(head.color)
    return mod