import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//--------------------------------------Search---------------------------------------------------------------------------------------
SearchResponse = WS.sendRequestAndVerify(findTestObject('OneWayTrip/1_Search Request'))

def slurper = new groovy.json.JsonSlurper()

def sSearchResponse = slurper.parseText(SearchResponse.getResponseBodyContent())

SearchToken = sSearchResponse.CatalogOfferingsResponse.CatalogOfferings.Identifier.value

GlobalVariable.sSearchToken = SearchToken

println('Value of Search Identifier Token is ' + GlobalVariable.sSearchToken)

WS.verifyResponseStatusCode(SearchResponse, 200)

//----------------------------------Price-------------------------------------------------------------------------------------------
PriceResponse = WS.sendRequestAndVerify(findTestObject('OneWayTrip/2_Price Request'))

def sPriceResponse = slurper.parseText(PriceResponse.getResponseBodyContent())

WS.verifyResponseStatusCode(PriceResponse, 200)

//println('Value of Search Identifier Token is ' + sPriceResponse.toString())
//-------------------------------------Initiate Workbench--------------------------------------------------------------------------
InitiateWorkBenchResponse = WS.sendRequest(findTestObject('OneWayTrip/3_TS Intiate_Workbench'))

def sInitiateWorkBench = slurper.parseText(InitiateWorkBenchResponse.getResponseBodyContent())

WorkBench = sInitiateWorkBench.ReservationResponse.Reservation.Identifier.value

GlobalVariable.reservationId = WorkBench

GlobalVariable.catalogOfferingsIdentifier = SearchToken

println('Value of WorkBench catalogOfferingsIdentifier ' + GlobalVariable.catalogOfferingsIdentifier)

println('Value of Reservation ' + GlobalVariable.reservationId)

WS.verifyResponseStatusCode(InitiateWorkBenchResponse, 200)

//-------------------------------------Add_Offer Catalogue-----------------------------------------------------------------------------
Add_Offer_Catalogue = WS.sendRequestAndVerify(findTestObject('OneWayTrip/4_Add_Offer Catalogue'))

WS.verifyResponseStatusCode(Add_Offer_Catalogue, 200)

//-----------------------------------------Add_Traveler------------------------------------------------------------------------------
GlobalVariable.reservationId = WorkBench

Add_Traveler = WS.sendRequestAndVerify(findTestObject('OneWayTrip/5_Add_Traveler'))

WS.verifyResponseStatusCode(Add_Traveler, 200)

//-----------------------------------------Commit--------------------------------------------------------------------------------------
Commit = WS.sendRequest(findTestObject('OneWayTrip/6_Commit', [('reservationId') : GlobalVariable.reservationId]))

WS.verifyResponseStatusCode(Commit, 200)

def sCommit = slurper.parseText(Commit.getResponseBodyContent())

sRecordLocator = sCommit.ReservationResponse.Reservation.Receipt[0].Confirmation.Locator.value

GlobalVariable.PNR = sRecordLocator

if (sRecordLocator.length() == 6) {
	
//println('The length of the PNR is ' + sRecordLocator.length())
//print('The Galileo Record Locator is '+ GlobalVariable.PNR)
WS.comment('The Galileo Record Locator is '+ GlobalVariable.PNR)
		
	
} else {
	
//println('The length of the PNR is ' + sRecordLocator.length())

WS.comment('The Galileo Record Locator is not generated')

}


//-----------------------------------------InitiateWorkBenchWithPNR---------------------------------------------------------------------
WorkBenchWithPNR = WS.sendRequest(findTestObject('OneWayTrip/7_Intiate workbench with PNR'))

WS.verifyResponseStatusCode(WorkBenchWithPNR, 200)

def sWorkBenchWithPNR = slurper.parseText(WorkBenchWithPNR.getResponseBodyContent())

sWorkBenchIdentifierWithPNR = sWorkBenchWithPNR.ReservationResponse.Identifier.value

//println('The WorkBenchIdentifierWithPNR is ' + sWorkBenchIdentifierWithPNR)
GlobalVariable.eCacheId = sWorkBenchIdentifierWithPNR

//-----------------------------------------Add_FoP---------------------------------------------------------------------------------------
AddFoP = WS.sendRequest(findTestObject('OneWayTrip/8_Add FOP'))

WS.verifyResponseStatusCode(AddFoP, 200)

//-----------------------------------------Add_Payment-----------------------------------------------------------------------------------
AddPayment = WS.sendRequest(findTestObject('OneWayTrip/9_Add Payment'))

WS.verifyResponseStatusCode(AddPayment, 200)

//----------------------------------------Ticket_Issuance---------------------------------------------------------------------------------
TicketIssuance = WS.sendRequest(findTestObject('OneWayTrip/R_Ticket Issuance'))

WS.verifyResponseStatusCode(TicketIssuance, 200)

def sTicketIssuance = slurper.parseText(TicketIssuance.getResponseBodyContent())

sTicketNumber = sTicketIssuance.ReservationResponse.Reservation.Receipt[3].Document[0].Number

GlobalVariable.sFirstTicket = sTicketNumber

if (sTicketNumber.length() == 13 ) {
	
println('The First Ticket is ' + GlobalVariable.sFirstTicket)


} else {

println('The Ticket is not Generated')

}

//-----------------------------------------GetEligibility-------------------------------------------------------------------------------------
Eligibility = WS.sendRequest(findTestObject('OneWayTrip/S_GetEligibilityRequest'))

WS.verifyResponseStatusCode(Eligibility, 200)

def sEligibility = slurper.parseText(Eligibility.getResponseBodyContent())

//sChange = ((sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].Penalties.Change[0])['@type'])
sExchangable = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].exchangeable

//println('The PNR is ' + sChange)
println('The exchangeable is ' + sExchangable)

//----------------------------------------ReservationWorkBench---------------------------------------------------------------------------------
ReservationWorkBench = WS.sendRequest(findTestObject('OneWayTrip/T_Reservationworkbench'))

WS.verifyResponseStatusCode(ReservationWorkBench, 200)

def sReservationWorkBench = slurper.parseText(ReservationWorkBench.getResponseBodyContent())

sIdentifierResWorkBench = sReservationWorkBench.ReservationResponse.Reservation.Identifier.value

GlobalVariable.IdentifierResWorkBench = sIdentifierResWorkBench

println('The IdentifierResWorkBench is ' + GlobalVariable.IdentifierResWorkBench)

//------------------------------------------SearchDomainPNR-------------------------------------------------------------------------------------
SearchDomainPNR = WS.sendRequestAndVerify(findTestObject('OneWayTrip/U-SearchDomainPNR'))

WS.verifyResponseStatusCode(SearchDomainPNR, 200)

def sSearchDomainPNR = slurper.parseText(SearchDomainPNR.getResponseBodyContent())

sExchSearchIdentifierValue = sSearchDomainPNR.CatalogOfferingsAirChangeResponse.CatalogOfferings.Identifier.value

GlobalVariable.ExchSearchIdentifierValue = sExchSearchIdentifierValue

println('The ExchSearchIdentifierValue is ' + GlobalVariable.ExchSearchIdentifierValue)

//--------------------------------------------Modify Offer----------------------------------------------------------------
ModifyOffer = WS.sendRequest(findTestObject('OneWayTrip/V_Modify Offer'))

WS.verifyResponseStatusCode(ModifyOffer, 200)

def sModifyOffer = slurper.parseText(ModifyOffer.getResponseBodyContent())

sOfferIdentifierValue = sModifyOffer.OfferListResponse.OfferID[0].Identifier.value

GlobalVariable.sOfferIdentifierValue = sOfferIdentifierValue

println('The OfferIdentifierValue is ' + GlobalVariable.sOfferIdentifierValue)

//--------------------------------------------Add Payment------------------------------------------------------------------
AddPayment = WS.sendRequest(findTestObject('OneWayTrip/X_Add Payment PNR'))

WS.verifyResponseStatusCode(AddPayment, 200)

//--------------------------------------------------------------------------------------------------------------------------
ReservationCommit = WS.sendRequest(findTestObject('OneWayTrip/Y_Reservation Commit'))

WS.verifyResponseStatusCode(ReservationCommit, 200)

def sReservationCommit = slurper.parseText(ReservationCommit.getResponseBodyContent())

sOldTicket = sReservationCommit.ReservationResponse.Reservation.Receipt[3].Document[0].Number

sNewTicket = sReservationCommit.ReservationResponse.Reservation.Receipt[4].Document[0].Number

GlobalVariable.sOldTicket = sOldTicket

GlobalVariable.sNewTicket = sNewTicket

WS.comment('The 1G Record Locator is ' + GlobalVariable.PNR)

if (sOldTicket.length() == 13 && sNewTicket.length() == 13) {
		
WS.comment('The Old Ticket is ' + GlobalVariable.sOldTicket)
	
WS.comment('The New Ticket is ' + GlobalVariable.sNewTicket)


	
} else {
	
WS.comment('Either The Old Ticket or The New Ticket is not Generated')

}
