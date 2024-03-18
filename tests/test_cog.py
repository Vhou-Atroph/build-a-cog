from direct.directbase.DirectStart import base

import build_a_cog as cog

yesman = cog.build(cog.Cog(suit=cog.Suit(model="tt_a_ene_cga_zero.bam",
        torso="phase_3.5/maps/c_blazer.jpg",
        arms="phase_3.5/maps/c_sleeve.jpg",
        legs="phase_3.5/maps/c_leg.jpg",
        hands=(0.95,0.75,0.75,1.0),
        sigil="CorpIcon"
    ),
    head=cog.Head(file="phase_4/models/char/suitA-heads.bam",
        node="yesman",
        texture=None,
        color=None
    ),
    animation=cog.Animation(file="phase_5/models/char/tt_a_ene_cga_song-and-dance.bam",
        anim_loop=True,
        loop_from=None,
        loop_to=None,
        loop_restart=None,
        pose=False,
        pose_frame=None
    )
))

yesman.setPos(0,20,-4.2)
yesman.setH(180)
yesman.reparentTo(render)

base.run()
