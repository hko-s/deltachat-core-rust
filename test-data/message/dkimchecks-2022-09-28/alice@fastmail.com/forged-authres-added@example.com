Authentication-Results: mx1.messagingengine.com;
    x-csa=none;
    x-me-sender=none;
    x-ptr=pass smtp.helo=nx184.node01.secure-mailgate.com
      policy.ptr=nx184.node01.secure-mailgate.com
Authentication-Results: mx1.messagingengine.com;
    bimi=skipped (DMARC did not pass)
Authentication-Results: mx1.messagingengine.com;
    arc=none (no signatures found)
Authentication-Results: mx1.messagingengine.com;
    dkim=none (no signatures found);
    dmarc=none policy.published-domain-policy=none
      policy.applied-disposition=none policy.evaluated-disposition=none
      (p=none,d=none,d.eval=none) policy.policy-from=p
      header.from=delta.blinzeln.de;
    iprev=pass smtp.remote-ip=89.22.108.184
      (nx184.node01.secure-mailgate.com);
    spf=none smtp.mailfrom=alice@delta.blinzeln.de
      smtp.helo=nx184.node01.secure-mailgate.com
From: forged-authres-added@example.com
Authentication-Results: aaa.com; dkim=pass header.i=@example.com
Authentication-Results: aaa.com; dkim=pass header.i=@example.com
