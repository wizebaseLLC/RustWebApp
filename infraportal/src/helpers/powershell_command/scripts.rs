pub fn remove_cgi_node(name: &str) -> String {
    format!(
        r#"$ErrorActionPreference= 'silentlycontinue';$session = New-PSSession PIPWITS101.nb.com -Name session;$remove = Invoke-Command -Session $session -ScriptBlock {{  Set-Location 'c:\\scripts';  return cmd.exe '/c del_node2 {}' }} -ErrorVariable i;[pscustomobject]@{{ results = $remove[2];  errors = $i[1].Exception.SerializedRemoteException.Message  }} | ConvertTo-Json;"#,
        name
    )
}
