from direct.actor.Actor import Actor
from direct.showbase.ShowBase import ShowBase  
from pandac import *
from direct import *
from panda3d.core import *
from direct.showbase.Loader import *

from . import rustycog

def _genhead(head:rustycog.Head):
    """Creates a generic cog head according to standard Cog heads."""
    mod = loader.loadModel(head.file).find("**/"+head.node)
    if head.texture:
        mod.setTexture(loader.loadTexture(head.texture),1)
    if head.color:
        mod.setColor(head.color)
    return mod

def build(cog:rustycog.Cog):
    actor = Actor(cog.suit.model,{"animation":cog.animation.file})
    actor.find('**/hands').setColor(cog.suit.hands)
    actor.findAllMatches('**/torso').setTexture(loader.loadTexture(cog.suit.torso),1)
    actor.findAllMatches('**/arms').setTexture(loader.loadTexture(cog.suit.arms),1)
    actor.findAllMatches('**/legs').setTexture(loader.loadTexture(cog.suit.legs),1)

    if cog.head:
        head = _genhead(cog.head)
        head.reparentTo(actor.find('**/def_head'))
    
    actor.play("animation")
    if cog.animation.anim_loop:
        actor.loop("animation",restart=cog.animation.loop_restart,fromFrame=cog.animation.loop_from,toFrame=cog.animation.loop_to)
    if cog.animation.pose:
        actor.pose("animation",cog.animation.pose_frame)
    return actor